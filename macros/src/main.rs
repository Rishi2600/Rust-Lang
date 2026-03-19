macro_rules! add{
 // first arm match add!(1,2), add!(2,3) etc
    ($a:expr,$b:expr)=>{
        {
            $a+$b
        }
    };
// Second arm macth add!(1), add!(2) etc
    ($a:expr)=>{
        {
            $a
        }
    }
}

fn main(){
    let x=0;
    let arm1 = add!(1,2);
    println!("{}",  arm1);
    let arm2 = add!(x);
    println!("{}", arm2);

    let final_sum = arm1 + arm2;

    println!("sum of both arms: {}", final_sum);
}

/*There can be multiple branches in a single macro expanding to different code based on different arguments.
Each branch can take multiple arguments, starting with the $ sign and followed by a token type:*/