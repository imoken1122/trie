use std::io::Error as Error;
use crate::array_trie::Trie;
use crate::VOCAB_SIZE;
use crate::idx_t;
type next_elm_t = i64;
type offset_elm_t = i64;
type offset_t = Option<offset_elm_t>;
type next_t= Option<next_elm_t>;
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

        // nextの初期値として、次の空き要素を負のindexとして表現. 
        // next[0] == -4 なら空き要素が4ということ
        let next = (1..VOCAB_SIZE+1).map(|i| Some(- (i as i64))).collect();
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

            // offset を計算
            let offset = self.get_offset(node_idx);

            // offsetを使ってそれぞれのarrayを更新
            self.update_arrays(offset,node_idx);

            // nextの一番左の空き要素のindexを更新
            self.min_empty_ni = self.get_left_idx_next(self.next.as_ref());

        }


    }
    fn get_left_idx_next(&self,array:&Vec<Option<next_elm_t>>, ) ->idx_t{
        for i in 0..array.len(){
            if array[i] < Some(0) {
                return i;
            }
        }
        panic!("not exist empty element in this array. {:?}",array);
    }
    fn get_left_node_idx(&self,array:&Vec<Option<idx_t>>, ) ->idx_t{
        for i in 0..array.len(){
            if array[i] != None{
                return i;
            }
        }
        panic!("not exist empty element in this array. {:?}",array);
    

    }
    fn check_next(&self, offset : offset_elm_t,node_idx : usize) -> bool{
        let children: &Vec<Option<idx_t>> = self.trie.nodes[node_idx].children.as_ref();

        //println!("{:?}",children);
        //全ての子ノードのindexにoffsetを加算したindexが全て nextの空いてる要素かをチェック
        for ci in 0..VOCAB_SIZE{

            if children[ci] == None { continue;}
            let idx = ci as offset_elm_t + offset ;
            if idx>= 0{
                if self.next[idx as usize] >= Some(0){ return false ;}
            }else{
                panic!("index is <0");

            }
        }
        return true;

    }
    fn get_offset(&mut self, node_idx : usize) -> offset_elm_t{
        //現在のnext配列が空要素である最小index
        let mut next_empty_ni = self.min_empty_ni;

        // childrenの一番左のノードのindexを取得
        let left_idx = self.get_left_node_idx(self.trie.nodes[node_idx].children.as_ref());

        while next_empty_ni < self.next.len(){

            //スライド幅を計算
            let tmp_offset = next_empty_ni as offset_elm_t - left_idx as offset_elm_t;

            //println!("{:?}",tmp_offset);
            // 計算したスライド幅で全ての子ノードをずらした時、nextの空いてる要素と対応するかチェック
            if self.check_next(tmp_offset, node_idx){
                return tmp_offset 
            }

            //次の next配列の空き要素のindexに遷移
            next_empty_ni = -self.next[next_empty_ni].unwrap() as usize ;

        }
        panic!("Not found offset. node_idx is {}",node_idx);

        
    }
    fn update_arrays(&mut self, offset : offset_elm_t, parent_idx: usize) {

        let children = self.trie.nodes[parent_idx].children.clone();

        self.offset[parent_idx] = Some(offset);

        // children
        for ci in 0..VOCAB_SIZE{
            if children[ci] == None { continue;}

            let idx = (ci as offset_elm_t + offset) as usize ;
            self.next[idx ] = Some(children[ci].unwrap() as i64);
            self.check[idx] = Some(parent_idx);
        }


    }
    fn verify_next(&self, ans : &[i64]) -> bool{
        let mut flag = true;
        for i in 0..ans.len(){
            if self.next[i].unwrap() != ans[i]{
                flag = false;
            }

        }
        for i in ans.len()..self.next.len(){
            if self.next[i].unwrap()>=0 {
                flag = false;
            }
        }
        flag

    }

}



mod test{
    use super::*;
    #[test]
    fn test_build(){

        let mut trie = Trie::new();
        trie.insert("ab".to_string());
        trie.insert("abc".to_string());
        trie.insert("cab".to_string());
        trie.insert("cba".to_string());
        trie.insert("dae".to_string());

        //println!("{:?}", trie.nodes);
        let mut ta = DoubleArray::new(trie);
        //println!("{:?}", ta.next);

        ta.build();

        let next_ans : [i64;11] = [1,2,4,9,3,5,7,6,8,10,11];
        assert_eq!(ta.verify_next(&next_ans),true);
        //println!("{:?}", ta.next);

        let mut trie = Trie::new();
        let mut word = ["ae","ace","cb"];
        word.sort();
        for w in word.iter(){
            trie.insert(w.to_string());
        }

        let mut ta = DoubleArray::new(trie);

        ta.build();

        //kjprintln!("{:?}", ta.next);
        println!("{:?}", ta.offset);
        let next_ans : [i64;6] = [1,3,2,4,5,6];
        assert_eq!(ta.verify_next(&next_ans),true);


    }
}
