mod tree;
mod node;
mod iter;


#[cfg(test)]
mod tests {
    use crate::tree::TreeLog;
    use crate::iter::TreeIterator;

    #[test]
    fn it_works() {
        let source = b"abcdefgh";

        let mut i = 0;

        let mut tree = TreeLog::new();

        println!("{:?}", tree);

        while i < source.len() {
            let chunk = String::from_utf8(Vec::from(&source[i..i + 2])).unwrap(); //TODO unwrap
            tree.log(chunk);
            i += 2;

            println!("{:?}", tree);
        }

        tree.print();

        for node in tree.iter() {
            println!("Next: {:?}", node);
        }

        println!("Audit {:?}", TreeIterator::generate_trail(&tree, "FB2B7FCE0940161406A6AA3E4D8B4AA6104014774FFA665743F8D9704F0EB0EC"));
        println!("Audit {:?}", TreeIterator::generate_trail(&tree, "21E21C35A5823FDB452FA2F9F0A612C74FB952E06927489C6B27A43B817BED4"));

        let client_trail = vec![
            "FB2B7FCE0940161406A6AA3E4D8B4AA6104014774FFA665743F8D9704F0EB0EC".to_string(),
            "C1E70A0150D82BF838E346D34BB993AC01A2A7D5FDBAA809D9485F37734E5005".to_string(),
            "C1E70A0150D82BF838E346D34BB993AC01A2A7D5FDBAA809D9485F37734E5005".to_string()

        ];

        println!("Audit log entry [gh]: {:?}", tree.audit("FB2B7FCE0940161406A6AA3E4D8B4AA6104014774FFA665743F8D9704F0EB0EC", &client_trail));
    }
}
