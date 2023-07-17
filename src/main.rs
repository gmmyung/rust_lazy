use rust_lazy::operation;

fn main() {
    let scalar1 = operation::Scalar::new(1.);
    let scalar2 = operation::Scalar::new(2.); 
    let scalar3 = operation::Scalar::new(3.);
    let scalar4 = operation::Scalar::new(4.);
    let scalar5 = operation::Scalar::new(5.);

    let add = scalar1.add(&scalar2);
    let sub = scalar3.sub(&scalar4);
    let mul = scalar5.mul(&add);

    let mulop = &add * &sub;
    let divop = &add / &mul;

    let res = &mulop + &divop;

    println!("{}", res);
    println!("result: {}", res.execute());
    res.compile().iter().for_each(|x| println!("{}", x)); 
}
