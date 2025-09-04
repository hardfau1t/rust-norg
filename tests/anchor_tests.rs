use rust_norg::{
    parse_tree, LinkTarget,
    LinkTarget::Heading,
    NorgAST::Paragraph,
    ParagraphSegment::{Anchor, AnchorDefinition, Link, Token},
    ParagraphSegmentToken::{Special, Text, Whitespace},
};
use test_log::test;

// Basic Anchor Declaration Tests
#[test]
fn test_anchor_declaration_basic() {
    let norg = r#"[Neorg] is a fancy organizational tool.

[Neorg]{https://github.com/nvim-neorg/neorg}"#;
    let result = parse_tree(norg).expect("Failed to parse basic anchor declaration");
    assert_eq!(result, vec![]);
    // Should create anchor reference and definition
    assert_eq!(
        result,
        vec![
            Paragraph(vec![
                Anchor {
                    content: vec![Token(Text("Neorg".to_string()))],
                    description: None
                },
                Token(Whitespace),
                Token(Text("is".to_string())),
                Token(Whitespace),
                Token(Text("a".to_string())),
                Token(Whitespace),
                Token(Text("fancy".to_string())),
                Token(Whitespace),
                Token(Text("organizational".to_string())),
                Token(Whitespace),
                Token(Text("tool".to_string())),
                Token(Special('.'))
            ]),
            Paragraph(vec![AnchorDefinition {
                content: vec![Token(Text("Neorg".to_string()))],
                target: Box::new(Link {
                    filepath: None,
                    targets: vec![LinkTarget::Url(
                        "https://github.com/nvim-neorg/neorg".to_string()
                    )],
                    description: None
                })
            }])
        ],
        "Failed to run test test_anchor_declaration_basic"
    );
}

#[test]
fn test_anchor_declaration_standalone() {
    let norg = r#"This text references [anchor name] in the content.

[anchor name]{* Target Heading}"#;
    let result = parse_tree(norg).expect("Failed to parse standalone anchor declaration");
    assert_eq!(result, vec![]);
    assert_eq!(
        result,
        vec![
            Paragraph(vec![
                Token(Text("This".to_string())),
                Token(Whitespace),
                Token(Text("text".to_string())),
                Token(Whitespace),
                Token(Text("references".to_string())),
                Token(Whitespace),
                Anchor {
                    content: vec![
                        Token(Text("anchor".to_string())),
                        Token(Whitespace),
                        Token(Text("name".to_string()))
                    ],
                    description: None
                },
                Token(Whitespace),
                Token(Text("in".to_string())),
                Token(Whitespace),
                Token(Text("the".to_string())),
                Token(Whitespace),
                Token(Text("content".to_string())),
                Token(Special('.'))
            ]),
            Paragraph(vec![AnchorDefinition {
                content: vec![
                    Token(Text("anchor".to_string())),
                    Token(Whitespace),
                    Token(Text("name".to_string()))
                ],
                target: Box::new(Link {
                    filepath: None,
                    targets: vec![Heading {
                        level: 1,
                        title: vec![
                            Token(Text("Target".to_string())),
                            Token(Whitespace),
                            Token(Text("Heading".to_string()))
                        ]
                    }],
                    description: None
                })
            }])
        ]
    );
}
#[test]
fn test_anchor_with_description() {
    let norg = r#"[anchor][custom description]

[anchor]{# target-location}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor with description");
    assert_eq!(result, vec![]);
    assert_eq!(
        result,
        vec![
            Paragraph(vec![Anchor {
                content: vec![Token(Text("anchor".to_string()))],
                description: Some(vec![
                    Token(Text("custom".to_string())),
                    Token(Whitespace),
                    Token(Text("description".to_string()))
                ])
            }]),
            Paragraph(vec![AnchorDefinition {
                content: vec![Token(Text("anchor".to_string()))],
                target: Box::new(Link {
                    filepath: None,
                    targets: vec![LinkTarget::Generic(vec![
                        Token(Text("target".to_string())),
                        Token(Special('-')),
                        Token(Text("location".to_string()))
                    ])],
                    description: None
                })
            }])
        ]
    )
}

// Anchor Definition Tests
#[test]
fn test_anchor_definition_heading() {
    let norg = r#"[section reference]{* Important Section}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor definition to heading");
    assert_eq!(result, vec![]);
    // Should link anchor to heading
    assert_eq!(
        result,
        vec![Paragraph(vec![AnchorDefinition {
            content: vec![
                Token(Text("section".to_string())),
                Token(Whitespace),
                Token(Text("reference".to_string()))
            ],
            target: Box::new(Link {
                filepath: None,
                targets: vec![Heading {
                    level: 1,
                    title: vec![
                        Token(Text("Important".to_string())),
                        Token(Whitespace),
                        Token(Text("Section".to_string()))
                    ]
                }],
                description: None
            })
        }])]
    );
}

#[test]
fn test_anchor_definition_external_url() {
    let norg = r#"[homepage]{https://example.com}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor definition to external URL");
    assert_eq!(result, vec![]);
    // Should link anchor to external URL
    assert_eq!(
        result,
        vec![Paragraph(vec![AnchorDefinition {
            content: vec![Token(Text("homepage".to_string()))],
            target: Box::new(Link {
                filepath: None,
                targets: vec![LinkTarget::Url("https://example.com".to_string())],
                description: None
            })
        }])]
    )
}

#[test]
fn test_anchor_definition_file_location() {
    let norg = r#"{:docs/setup:}[documentation]"#;
    let result = parse_tree(norg).expect("Failed to parse anchor definition to file location");
    assert_eq!(
        result,
        vec![Paragraph(vec![AnchorDefinition {
            content: vec![Token(Text("documentation".to_string()))],
            target: Box::new(Link {
                filepath: Some("docs/setup".to_string()),
                targets: vec![],
                description: None
            })
        }])]
    );
}

/*
// Multiple Anchor Usage
#[test]
fn test_multiple_anchor_declarations() {
    let norg = r#"Both [first] and [second] are useful resources.

[first]{https://first.example.com}
[second]{https://second.example.com}"#;
    let result = parse_tree(norg).expect("Failed to parse multiple anchor declarations");
    assert_eq!(result, vec![]);
    // Should handle multiple anchors
}

#[test]
fn test_repeated_anchor_usage() {
    let norg = r#"First mention of [tool] and second mention of [tool].

[tool]{https://tool.example.com}"#;
    let result = parse_tree(norg).expect("Failed to parse repeated anchor usage");
    assert_eq!(result, vec![]);
    // Should link both instances to the same target
    let link_count = result.matches("href=\"https://tool.example.com\"").count();
}

// Anchor URL Mapping Tests (Based on links-mapping.txt)
#[test]
fn test_anchor_url_mapping() {
    let norg = r#"Reference to [Neorg] in text.

[Neorg]{# target-location}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor URL mapping");
    assert_eq!(result, vec![]);
    // Should map to: http://127.0.0.1/view/<current_path>#Neorg
             result.contains("href=\"#Neorg\""));
}

#[test]
fn test_anchor_current_path_mapping() {
    let norg = r#"Link to [local-ref] within document.

[local-ref]{# local-reference}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor current path mapping");
    assert_eq!(result, vec![]);
    // Should generate current path URL mapping
}

// Anchor Integration with Other Features
#[test]
fn test_anchor_in_paragraph() {
    let norg = r#"This paragraph contains [inline anchor] for reference.

[inline anchor]{https://example.com/reference}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor in paragraph");
    assert_eq!(result, vec![]);
    // Should work within paragraph text
}

#[test]
fn test_anchor_in_list() {
    let norg = r#"- First item with [list anchor]
- Second item
- Third item

[list anchor]{* Target Section}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor in list");
    assert_eq!(result, vec![]);
    // Should work within list items
}

#[test]
fn test_anchor_in_heading() {
    let norg = r#"* Heading with [heading anchor] reference

[heading anchor]{https://example.com}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor in heading");
    assert_eq!(result, vec![]);
    // Should work within headings
}

// Complex Anchor Scenarios
#[test]
fn test_anchor_with_attached_modifiers() {
    let norg = r#"This is *[important anchor]* text.

[important anchor]{# important-section}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor with attached modifiers");
    assert_eq!(result, vec![]);
    // Should work within attached modifiers
}

#[test]
fn test_anchor_forward_reference() {
    let norg = r#"Forward reference to [future anchor].

Later in document...

[future anchor]{* Later Section}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor forward reference");
    assert_eq!(result, vec![]);
    // Should handle forward references
}

#[test]
fn test_anchor_backward_reference() {
    let norg = r#"[early anchor]{* Early Section}

Later reference to [early anchor]."#;
    let result = parse_tree(norg).expect("Failed to parse anchor backward reference");
    assert_eq!(result, vec![]);
    // Should handle backward references
}

// Anchor HTML Generation
#[test]
fn test_anchor_html_structure() {
    let norg = r#"Link to [structured anchor] here.

[structured anchor]{https://structured.example.com}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor HTML structure");
    assert_eq!(result, vec![]);
    // Should generate proper HTML anchor structure
}

#[test]
fn test_anchor_css_classes() {
    let norg = r#"Text with [classed anchor] link.

[classed anchor]{# class-target}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor CSS classes");
    assert_eq!(result, vec![]);
    // Should have appropriate CSS classes
             result.contains("anchor") ||
             result.contains("href="));
}

#[test]
fn test_anchor_accessibility() {
    let norg = r#"Accessible [screen reader anchor] link.

[screen reader anchor]{https://accessible.example.com}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor accessibility");
    assert_eq!(result, vec![]);
    // Should have accessibility attributes
    // May have additional accessibility attributes
}

// Special Characters and Edge Cases
#[test]
fn test_anchor_special_characters() {
    let norg = r#"Link with [special & chars!] in name.

[special & chars!]{# special-target}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor with special characters");
    assert_eq!(result, vec![]);
    // Should handle special characters
}

#[test]
fn test_anchor_unicode_characters() {
    let norg = r#"Reference to [café anchor] here.

[café anchor]{https://café.example.com}"#;
    let result = parse_tree(norg).expect("Failed to parse anchor with unicode");
    assert_eq!(result, vec![]);
    // Should handle unicode characters
}

#[test]
fn test_anchor_multiword_names() {
    let norg = r#"Link to [multi word anchor name] reference.

[multi word anchor name]{* Target Section}"#;
    let result = parse_tree(norg).expect("Failed to parse multiword anchor names");
    assert_eq!(result, vec![]);
    // Should handle multi-word anchor names
}

// Error Handling
#[test]
fn test_anchor_undefined_reference() {
    let norg = r#"Reference to [undefined anchor] without definition."#;
    let result = parse_tree(norg).expect("Failed to parse undefined anchor reference");
    assert_eq!(result, vec![]);
    // Should handle undefined anchors gracefully
    // May show as broken link or plain text
}

#[test]
fn test_anchor_definition_without_reference() {
    let norg = r#"[orphan anchor]{https://orphan.example.com}"#;
    let result = parse_tree(norg).expect("Failed to parse orphan anchor definition");
    assert_eq!(result, vec![]);
    // Should handle orphaned definitions gracefully
}

#[test]
fn test_anchor_circular_reference() {
    let norg = r#"[circular1]{# circular2}
[circular2]{# circular1}"#;
    let result = parse_tree(norg).expect("Failed to parse circular anchor reference");
    assert_eq!(result, vec![]);
    // Should handle circular references gracefully
}
*/
