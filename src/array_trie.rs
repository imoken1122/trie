
// 部分文字列パターンと全ての文字のテーブルによるTrie 
//
// example: abc,acd
// Table :
//     a | -1,1,4,..,-1
//     ab| -1,-1,3,..,-1
//     abc| -1,-1,...-1
//     ac | -1,-1,-1,5,..,-1
//     acd |-1,-1,...,-1
//


use crate::VOCAB_SIZE;
#[derive(Debug,PartialEq)]
pub struct TrieNode {
    pub item : Option<String>,
    pub children  : Vec<Option<usize>>,
    pub num_child : usize
}

impl TrieNode {
    pub fn new(item:Option<String>) -> TrieNode{
        TrieNode {
            item :item ,
            children : vec![None;VOCAB_SIZE],
            num_child : 0

        }

    }

}


#[derive(Debug,PartialEq)]
pub struct Trie{
    pub nodes : Vec<TrieNode>
}

impl Trie{
    pub fn new() -> Trie{
        let root = TrieNode::new(None);
        let nodes = vec![root];
        Trie{
            nodes 
        }
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
                // 子ノードの数を追加
                self.nodes[node_idx].num_child += 1;
            }
            // 次のノードに遷移 
            node_idx = next_node_idx.unwrap();

            self.nodes[node_idx].item = Some(word[0..i+1].to_string());
        }


        Ok(())

    }


    pub fn contain(&self, word : String) -> bool {
        let mut node_idx = 0; 
        for (i,c)  in word.chars().enumerate(){
            // 'a'を基準に文字をindexに変換
            let w_idx = c as usize - 'a' as usize;
            let mut next_node_idx = self.nodes[node_idx].children[w_idx];
            if node_idx!=0 && self.nodes[node_idx].item.as_ref().unwrap() == &word { return true }
            // 登録されていれば false
            if next_node_idx == None { return false }
            // 次のノードに遷移 
            node_idx = next_node_idx.unwrap();
        }
        
        if node_idx!=0 && self.nodes[node_idx].item.as_ref().unwrap() == &word { return true }
        false
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
        println!("{:?}",trie.nodes);
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

    #[test]
    fn test_contain(){
        let mut trie = Trie::new();
        trie.insert("above".to_string());
        trie.insert("about".to_string());
        trie.insert("abs".to_string());
        trie.insert("char".to_string());
        assert_eq!( trie.contain("above".to_string()),true);
        assert_eq!( trie.contain("ab".to_string()),true);
        assert_eq!( trie.contain("aov".to_string()),false);
        assert_eq!( trie.contain("char".to_string()),true);
        assert_eq!( trie.contain("like".to_string()),false);
    

    }
}
