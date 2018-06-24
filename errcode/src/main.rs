//#[derive(Copy, Clone)] //well, this is using the copy and move constructor, but it won't work
struct Person {
    name : Option<String>,
    birth : i32,
}

//unwrap is the method to panic if Option is actually None

fn main() {
    //rust references are more like pointers, but it has implicit dereferencing
    // abilities, for example
    let mut v = vec![210,11,22];
    //implicit dereferenced.
    v.sort();
    //you can also do this, but much uglier
    (*v).sort();
    //and this is how you create a mut reference
//    let refv = &mut v;


    let mut composers = Vec::new();
    composers.push(Person {name : Some("Palestrina".to_string()), birth : 32});
    //this is okay
    let first_person = composers[0].name.take();
    //this is not
    //let first_person = composers[0].name;

    //this can also work
    //unwrap works on Option, panic if it is none
    //let first_person = composers.pop().unwrap();


    let v = vec![3,4,4,5,];
    println!("{:?}", v[1000]);
    //this won't work out
    // let r;
    // {
    //     let x = 1;
    //     r = &x;
    // }


}

//yes, I remember when we had this problem,
struct Tree<'a> {
    left : &'a Tree<'a>,
    right: &'a Tree<'a>,
    data : i32
}
//in this case, since you will never know the lifetime of your child, you might
// as well use reference count, another solution is using index instead of pointers


static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe {
        //use mutable static variable requires static, since anythread can access it
        STASH = p;
    }
}
