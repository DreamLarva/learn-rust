// 1,2,3 返回 3
macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    // 递归 算数量,
    ($head:expr $(, $tail:expr)*) => (1 + count_exprs!($($tail),*));
}
// 目标
// 实现一个
// let fib = recurrence![a[n] = 0, 1, ..., a[n-1] + a[n-2]];
// for e in fib.take(10) { println!("{}", e) }
macro_rules! recurrence {
    // ident:标识符 ty:类型 expr:表达式
    //  $($inits:expr),+ 加号作用于 $($inits:expr),
    // 为什么 用 ; 分隔 因为 expr 不能包含 ; 所以将 expr 和 ... 分隔开来了`
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ; ... ; $recur:expr ) => {
        {
            use std::ops::Index;

            const MEM_SIZE: usize = count_exprs!($($inits),+);

            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }

            // 实现 index 就可以 使用 [index] 来取元素
            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                // inline 优化
                #[inline(always)]
                fn index(&self, index: usize) -> &$sty { // 生命周期省略 第三条规则
                // fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;
                    // Wrapping 保证数值不会溢出

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = $sty;

                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                            $recur // a[n-1] + a[n-2]]
                        };

                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}

pub fn main() {
    let fib = recurrence![a[n]: u64 = 0,1; ...; a[n-1] + a[n-2]];
    for e in fib.take(10) {
        println!("{}", e);
    }
}
