use egg::{rewrite as rw, *};
use egg_latin::{RecExpr, POS};

fn main() {
    let expr: RecExpr = "(sentence (sub ?s) (d_obj ?do) (verb ?v))"
        .parse::<RecExpr>()
        .unwrap();
    println!("Testing expr: {}", expr);

    struct MyCostFn;
    impl CostFunction<POS> for MyCostFn {
        type Cost = f64;
        fn cost<C>(&mut self, enode: &POS, mut costs: C) -> Self::Cost
        where
            C: FnMut(Id) -> Self::Cost,
        {
            use POS::*;
            let op_cost = match enode {
                Emphasis(_) => 1.0,
                _ => 100.0,
            };
            enode.fold(op_cost, |sum, id| sum + costs(id))
        }
    }

    // Some sort of statistical analysis needed to find most important word in sentence based on
    // context

    let rules: &[Rewrite<POS, ()>] = &[
        rw!("sov"; "(sentence (sub ?s) (verb ?v) (d_obj ?do))" => "(sentence (sub ?s) (d_obj ?do) (verb ?v))"),
        rw!("osv"; "(sentence (sub ?s) (emp (d_obj ?do)) (verb ?v))" => "(sentence (d_obj ?do) (sub ?s) (verb ?v))"),
        rw!("vso"; "(sentence (sub ?s) (d_obj ?do) (emp (verb ?v)))" => "(sentence (verb ?v) (sub ?s) (d_obj ?do))"),
    ];

    let start = "(sentence (sub puella) (d_obj canem) (verb amat))"
        .parse()
        .unwrap();

    let start_2 = "(sentence (sub puella) (emp (d_obj canem)) (verb amat))"
        .parse()
        .unwrap();

    println!("Cost of start: {}", MyCostFn.cost_rec(&start));
    println!("Cost of start_2: {}", MyCostFn.cost_rec(&start_2));

    let runner = Runner::default().with_expr(&start).run(rules);

    let extractor = Extractor::new(&runner.egraph, MyCostFn);
    let (best_cost_sentence, best_expr_sentence) = extractor.find_best(runner.roots[0]);

    println!("best_cost_sentence: {}", best_cost_sentence);
    println!("best_expr_sentence: {}", best_expr_sentence);

    let runner_2 = Runner::default().with_expr(&start_2).run(rules);
    let extractor_2 = Extractor::new(&runner_2.egraph, MyCostFn);
    let (best_cost_sentence_2, best_expr_sentence_2) = extractor_2.find_best(runner_2.roots[0]);

    println!("best_cost_sentence_2: {}", best_cost_sentence_2);
    println!("best_expr_sentence_2: {}", best_expr_sentence_2);
}
