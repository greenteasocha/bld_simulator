memo: Wrong state ではなくて Alternative state としたいかな

Rust のジェネリクスについて知りたい。
例えば 3x3 operations を
{
    CornerSwap, CornerTwist, EdgeSwap, EdgeFlip
}
と定義したとき、func(3x3Operations) を optionFunc 的に受け取れる関数を作ったとして、func(CornerSwap) をそこに入れることができる？

enum Operation3x3 {
    CornerSwap,
    CornerTwist,
    EdgeSwap,
    EdgeFlip,
}
として、nearby_search<T> 的な感じにして、func(T) → T をたくさん受け取れるようにしたい。

例えば nearby_search<3x3Operation> であれば複数の func(3x3Operation) → vec(3x3Operation)を受け取れるようにしたい。

以下のようになる。

impl NearbyMixedOperationSearch<T> {
    pub fn explore_variants_two_changes(
        &self,
        initial_state: &State,
    ) -> Vec<(ModifiedMixedSequence, State)> {
        let variants_for_each_step: Vec<Vec<T>> = self.base_operations.iter()
            .map(|op| self.get_alternative_operations(op))
            .collect();
        for step1 in 0..self.base_operations.len() {
            for step2 in (step1 + 1)..self.base_operations.len() {
                
                for alt_op1 in &variants_for_each_step[step1] {
                    for alt_op2 in &variants_for_each_step[step2] {
                        let mut modified = ModifiedMixedSequence::new(self.base_operations.clone());
                        modified.add_modifier(alt_op1.clone());
                        modified.add_modifier(alt_op2.clone());

                        let final_state = self.apply_modified_sequence(initial_state, &modified_sequence);

                        variants.push((modified_sequence, final_state));
                    }
                }
            }
        }
    }

    fn get_alternative_operations(&self, operation: &T) -> Vec<T> {
        // T に基づいて代替操作を生成するロジックを実装
        for self.alternative_generators {
            alternative_ops.extend(generator(operation));
        }
        alternative_ops
    }
}

こういう風にして、 alternative_generator の生成を注入可能にしてほしい。
なお、 alternative_generator の例は以下のようなものがある

func CornerSwapAlternatives(op: 3x3Operation) -> vec(3x3Operation) {
    // 例えば CornerSwap に対して、同じ位置の CornerTwist を返す
    match op {
        Operation3x3::CornerSwap => {
             use crate::inspection::CornerSwapOperation;
        let mut alternatives = Vec::new();

        const CORNER_BUFFER: usize = 2; // UFR

        // 各コーナー位置について
        for target in 0..8 {
            if target == CORNER_BUFFER {
                continue;
            }

            // 各方向について
            for orientation in 0..3 {
                alternatives.push(CornerSwapOperation::new(CORNER_BUFFER, target, orientation));
            }
        }

        alternatives
        },
        _ => vec![],
    }
}

CornerTwist, EdgeSwap, EdgeFlip に対しても既存の実装を参考に alternative_generator を用意しておき、NearbyMixedOperationSearch の生成時に注入する感じで。