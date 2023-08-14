use std::io::Error as Error;
use crate::array_trie::Trie;
use crate::VOCAB_SIZE;


type idx_t= usize;
type offset_t = Option<idx_t>;
type next_t= Option<idx_t>;
type check_t= Option<idx_t>;

#[derive(Debug,PartialEq)]
pub struct DoubleArray{
    next : Vec<next_t>, // 子ノードを保管, 空要素は、次の空要素のインデックスが格納
    check : Vec<check_t>, // 親ノードを保管
    offset: Vec<offset_t>, // 行ごとのスライド幅
    min_empty_ni : usize,      // argmin(next == -1)
    trie : Trie,    // 行列で構築ずみのトライ
}


impl DoubleArray {
    pub fn new(trie:Trie, ) -> DoubleArray{
        let next = vec![None;VOCAB_SIZE];
        let check= vec![None;VOCAB_SIZE];
        let offset = vec![None; trie.nodes.len()];
        let min_empty_ni = 0;
        DoubleArray{
            next,
            check,
            offset,
            min_empty_ni,
            trie
        }

    }
    pub fn build(&mut self){
        // 行列で表現したtrieを行(部分文字ノード)ごとに適当にスライドして１次元配列に潰す(Next配列)
        //そのスライド幅をoffsetに保存する
        // check配列は、インデックスが子ノードとして、要素には親ノードのインデックスを保存する
        
        for node_idx in 0..self.trie.nodes.len(){

            // children が存在しなければスキップ
            if self.trie.nodes[node_idx].num_child == 0 {continue;}

            let offset = self.get_offset(node_idx);

            self.update_arrays(offset);



        }


    }
    fn get_left_node_idx(&self, node_idx : usize) ->idx_t{
        for i in 0..VOCAB_SIZE{
            if self.trie.nodes[node_idx].children[i] != None{
                return i;
            }
        }
        panic!("Not exist children in node. node_idx is {}",node_idx);
    

    }
    fn check_next(&self, offset : usize ,node_idx : usize) -> bool{
        let children = self.trie.nodes[node_idx].children.clone();

        //全ての子ノードのindexにoffsetを加算したindexが全て nextの空いてる要素かをチェック
        for ci in 0..VOCAB_SIZE{

            if children[ci] == None { continue;}

            if self.next[ci + offset] != None { return false ;}
        }
        return true;

    }
    fn get_offset(&mut self, node_idx : usize) -> idx_t{
        //現在のnext配列が空要素である最小index
        let mut next_empty_ni :usize= self.next[self.min_empty_ni].unwrap() as usize;

        // childrenの一番左のノードのindexを取得
        let left_idx = self.get_left_node_idx(node_idx);

        while next_empty_ni < self.next.len(){

            //スライド幅を計算
            let tmp_offset = next_empty_ni - left_idx;

            // 計算したスライド幅で全ての子ノードをずらした時、nextの空いてる要素と対応するかチェック
            if self.check_next(tmp_offset, node_idx){
                return tmp_offset
            }

            //次の next配列の空き要素のindexに遷移
            next_empty_ni = self.next[next_empty_ni].unwrap() as usize ;

        }
        panic!("Not found offset. node_idx is {}",node_idx);

        
    }
    fn update_arrays(&mut self, offset : idx_t) {

    }

}
