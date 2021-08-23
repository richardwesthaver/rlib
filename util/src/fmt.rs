pub use text_trees;

#[test]
fn text_tree() {
  use text_trees::StringTreeNode;
  let tt = StringTreeNode::with_child_nodes(
    "Home".to_string(),
    vec![
      StringTreeNode::with_child_nodes(
        "".to_string(),
        vec![StringTreeNode::with_child_nodes(
          "Child 2".to_string(),
          vec![StringTreeNode::with_child_nodes(
            "Grand Child 2".to_string(),
            vec![StringTreeNode::with_children(
              "Great Grand Child 2".to_string(),
              vec!["Great Great Grand Child 2".to_string()].into_iter(),
            )]
            .into_iter(),
          )]
          .into_iter(),
        )]
        .into_iter(),
      ),
      StringTreeNode::with_children("Posts".to_string(), vec!["Child 3".to_string()].into_iter()),
    ]
    .into_iter(),
  );

  let tt_string = "Home\n+-- \n|   '-- Child 2\n|       '-- Grand Child 2\n|           '-- Great Grand Child 2\n|               '-- Great Great Grand Child 2\n'-- Posts\n    '-- Child 3\n";

  assert_eq!(tt.to_string(), tt_string);
}
