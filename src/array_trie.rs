

const VOCAB_SIZE : usize = 128; // ascii code

#[derive(Debug)]
pub struct TrieNode {
    item : Option<String>,
    children  : Vec<Option<usize>>
}

impl TrieNode {
    pub fn new(item:Option<String>) -> TrieNode{
        TrieNode {
            item :item ,
            children : vec![None;VOCAB_SIZE]
        }

    }

}

#[derive(Debug)]
pub struct Trie{
    nodes : Vec<TrieNode>
}

impl Trie{
    pub fn new() -> Trie{
        let root = TrieNode::new(None);
        let nodes = vec![root];
        Trie{
            nodes 
        }
    }
    fn norm_ascii(self, w : char) -> usize{
        w as usize - 'a' as usize

    }
    fn add_node(&mut self, node : TrieNode) -> Option<usize>{
        self.nodes.push(node);
        Some(self.nodes.len() - 1)

    }
    pub fn insert(&mut self,word : String) -> Result<(),()>{

        let mut node_idx = 0;
        for (i,c)  in word.chars().enumerate(){
            // 'a'を基準に文字をindexに変換
            let w_idx = c as usize - 'a' as usize;
            // 現在の node の w_idxに対応する子ノードidxを取得
            let mut next_node_idx = self.nodes[node_idx].children[w_idx];
            // 登録されていれば登録
            if next_node_idx == None {

                let next_node = TrieNode::new(None);
                // 新たなノードを追加し、次のノードindexを取得
                next_node_idx = self.add_node(next_node) ;
                // 現在の node の w_idxに対応する子ノードidxを登録
                self.nodes[node_idx].children[w_idx] = next_node_idx;
            }
            // 次のノードに遷移 
            node_idx = next_node_idx.unwrap();

            self.nodes[node_idx].item = Some(word[0..i+1].to_string());
        }


        Ok(())

    }
}

mod test{
    use super::*;
    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("above".to_string());
        trie.insert("about".to_string());
        trie.insert("abs".to_string());
        //println!("{:?}",trie.nodes);
        let nodes=trie.nodes;



        let word_idx = [0,1,14,21,4]; //above
        for i in 0..5{
            assert_eq!( nodes[i].children[word_idx[i]],Some(i+1)); //a
        }

        let word_idx = [ 20, 19]; //ut
        assert_eq!( nodes[3].children[word_idx[0]],Some(6)); 
        assert_eq!( nodes[6].children[word_idx[1]],Some(7));


        //abs
        assert_eq!( nodes[2].children[18],Some(8));
    }
}
