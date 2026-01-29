macro_rules! my_vec {
    ()=> (
           Vec::new()
    );
    ($($x:expr),*$(,)?) => {
        {
            let mut v  = Vec::new();

            $(
                v.push($x);
            )*
            v
        }
    };

    ($elem: expr; $count:expr) => {
        {
            let mut v = Vec::new();
            v.resize($count, $elem);
            v
        }


    }
}
fn main() {
    let new_vec: Vec<String> = my_vec![];
    let new_vec1 = my_vec![34, 32, 45];
    let new_vec2 = my_vec![34, 32, 45,];
    let new_vec3 = my_vec![foo()];
    let new_vec4 = my_vec![10;4];
    println!("{:?}", new_vec4);

    
}

fn foo() {}
