use criterion::{criterion_group, criterion_main, Criterion};
mod blop;

fn solve_fast() {


    let input_string = "
    12. ..6 7..
    ..3 ..8 ...
    ... ... ...
    
    ... ..1 ...
    ... 78. ...
    .9. ... ...
    
    .3. ... .2.
    .7. .9. ..8
    2.. 3.. 5.1
    ";

   let mut s1 = blop::Soduko::new(input_string);
   s1.solve_fast();
   //println!("s1 is {} \ns2 is {}", s1, s2);

}

fn solve_slow() {


    let input_string = "
    12. ..6 7..
    ..3 ..8 ...
    ... ... ...
    
    ... ..1 ...
    ... 78. ...
    .9. ... ...
    
    .3. ... .2.
    .7. .9. ..8
    2.. 3.. 5.1
    ";

   let mut s2 = blop::Soduko::new(input_string);
   s2.solve();
   //println!("s1 is {} \ns2 is {}", s1, s2);

}



fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve_fast", |b| b.iter(|| {solve_fast()}));
}

fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("solve_slow", |b| b.iter(|| {solve_slow()}));
}


criterion_group!(benches, criterion_benchmark,criterion_benchmark2);
criterion_main!(benches);