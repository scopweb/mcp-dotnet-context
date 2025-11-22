use anyhow::Result;
use std::path::Path;
use tree_sitter::{Node, Parser};

use crate::types::{CSharpFile, ClassInfo, InterfaceInfo, MethodInfo, PropertyInfo};

#[allow(dead_code)]
pub struct CSharpAnalyzer {
    parser: Parser,
}

#[allow(dead_code)]
impl CSharpAnalyzer {
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        let language = tree_sitter_c_sharp::language();
        parser.set_language(language)?;

        Ok(Self { parser })
    }

    pub fn analyze_file(&mut self, path: &Path) -> Result<CSharpFile> {
        let source = std::fs::read_to_string(path)?;
        let tree = self
            .parser
            .parse(&source, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse file"))?;

        let root = tree.root_node();

        // Extract namespace
        let namespace = self.extract_namespace(&root, &source);

        // Extract using directives
        let usings = self.extract_usings(&root, &source);

        // Extract classes
        let classes = self.extract_classes(&root, &source);

        // Extract interfaces
        let interfaces = self.extract_interfaces(&root, &source);

        Ok(CSharpFile {
            path: path.to_path_buf(),
            namespace,
            usings,
            classes,
            interfaces,
        })
    }

    fn extract_namespace(&self, node: &Node, source: &str) -> Option<String> {
        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            if child.kind() == "namespace_declaration"
                || child.kind() == "file_scoped_namespace_declaration"
            {
                // Find the name node
                let mut child_cursor = child.walk();
                for name_child in child.children(&mut child_cursor) {
                    if name_child.kind() == "qualified_name" || name_child.kind() == "identifier" {
                        return Some(name_child.utf8_text(source.as_bytes()).ok()?.to_string());
                    }
                }
            }
        }

        None
    }

    fn extract_usings(&self, node: &Node, source: &str) -> Vec<String> {
        let mut usings = Vec::new();
        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            if child.kind() == "using_directive" {
                let mut child_cursor = child.walk();
                for using_child in child.children(&mut child_cursor) {
                    if using_child.kind() == "qualified_name" || using_child.kind() == "identifier"
                    {
                        if let Ok(text) = using_child.utf8_text(source.as_bytes()) {
                            usings.push(text.to_string());
                        }
                    }
                }
            }
        }

        usings
    }

    fn extract_classes(&self, node: &Node, source: &str) -> Vec<ClassInfo> {
        let mut classes = Vec::new();
        self.walk_for_classes(node, source, &mut classes);
        classes
    }

    fn walk_for_classes(&self, node: &Node, source: &str, classes: &mut Vec<ClassInfo>) {
        if node.kind() == "class_declaration" {
            if let Some(class_info) = self.parse_class(node, source) {
                classes.push(class_info);
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_for_classes(&child, source, classes);
        }
    }

    fn parse_class(&self, node: &Node, source: &str) -> Option<ClassInfo> {
        let mut name = String::new();
        let mut modifiers = Vec::new();
        let mut methods = Vec::new();
        let mut properties = Vec::new();

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                "modifier" => {
                    if let Ok(text) = child.utf8_text(source.as_bytes()) {
                        modifiers.push(text.to_string());
                    }
                }
                "identifier" => {
                    if let Ok(text) = child.utf8_text(source.as_bytes()) {
                        name = text.to_string();
                    }
                }
                "declaration_list" => {
                    // Extract methods and properties from class body
                    self.extract_members(&child, source, &mut methods, &mut properties);
                }
                _ => {}
            }
        }

        if !name.is_empty() {
            Some(ClassInfo {
                name,
                modifiers,
                base_class: None,
                interfaces: vec![],
                methods,
                properties,
            })
        } else {
            None
        }
    }

    fn extract_members(
        &self,
        node: &Node,
        source: &str,
        methods: &mut Vec<MethodInfo>,
        properties: &mut Vec<PropertyInfo>,
    ) {
        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                "method_declaration" => {
                    if let Some(method) = self.parse_method(&child, source) {
                        methods.push(method);
                    }
                }
                "property_declaration" => {
                    if let Some(property) = self.parse_property(&child, source) {
                        properties.push(property);
                    }
                }
                _ => {}
            }
        }
    }

    fn parse_method(&self, node: &Node, source: &str) -> Option<MethodInfo> {
        let mut name = String::new();
        let mut return_type = String::new();
        let mut modifiers = Vec::new();
        let mut is_async = false;

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                "modifier" => {
                    if let Ok(text) = child.utf8_text(source.as_bytes()) {
                        if text == "async" {
                            is_async = true;
                        }
                        modifiers.push(text.to_string());
                    }
                }
                "identifier" => {
                    if let Ok(text) = child.utf8_text(source.as_bytes()) {
                        if name.is_empty() {
                            name = text.to_string();
                        }
                    }
                }
                "predefined_type" | "qualified_name" | "generic_name" => {
                    if let Ok(text) = child.utf8_text(source.as_bytes()) {
                        if return_type.is_empty() {
                            return_type = text.to_string();
                        }
                    }
                }
                _ => {}
            }
        }

        if !name.is_empty() {
            Some(MethodInfo {
                name,
                return_type: if return_type.is_empty() {
                    "void".to_string()
                } else {
                    return_type
                },
                parameters: vec![],
                modifiers,
                is_async,
            })
        } else {
            None
        }
    }

    fn parse_property(&self, node: &Node, source: &str) -> Option<PropertyInfo> {
        let mut name = String::new();
        let mut prop_type = String::new();
        let mut has_getter = false;
        let mut has_setter = false;

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    if let Ok(text) = child.utf8_text(source.as_bytes()) {
                        if name.is_empty() {
                            name = text.to_string();
                        }
                    }
                }
                "predefined_type" | "qualified_name" | "generic_name" => {
                    if let Ok(text) = child.utf8_text(source.as_bytes()) {
                        if prop_type.is_empty() {
                            prop_type = text.to_string();
                        }
                    }
                }
                "accessor_list" => {
                    // Check for get/set accessors
                    let mut acc_cursor = child.walk();
                    for accessor in child.children(&mut acc_cursor) {
                        if accessor.kind() == "accessor_declaration" {
                            if let Ok(text) = accessor.utf8_text(source.as_bytes()) {
                                if text.contains("get") {
                                    has_getter = true;
                                }
                                if text.contains("set") {
                                    has_setter = true;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if !name.is_empty() && !prop_type.is_empty() {
            Some(PropertyInfo {
                name,
                prop_type,
                has_getter,
                has_setter,
            })
        } else {
            None
        }
    }

    fn extract_interfaces(&self, node: &Node, source: &str) -> Vec<InterfaceInfo> {
        let mut interfaces = Vec::new();
        self.walk_for_interfaces(node, source, &mut interfaces);
        interfaces
    }

    fn walk_for_interfaces(&self, node: &Node, source: &str, interfaces: &mut Vec<InterfaceInfo>) {
        if node.kind() == "interface_declaration" {
            if let Some(interface_info) = self.parse_interface(node, source) {
                interfaces.push(interface_info);
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_for_interfaces(&child, source, interfaces);
        }
    }

    fn parse_interface(&self, node: &Node, source: &str) -> Option<InterfaceInfo> {
        let mut name = String::new();
        let mut methods = Vec::new();

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    if let Ok(text) = child.utf8_text(source.as_bytes()) {
                        name = text.to_string();
                    }
                }
                "declaration_list" => {
                    let mut properties = Vec::new();
                    self.extract_members(&child, source, &mut methods, &mut properties);
                }
                _ => {}
            }
        }

        if !name.is_empty() {
            Some(InterfaceInfo { name, methods })
        } else {
            None
        }
    }
}
