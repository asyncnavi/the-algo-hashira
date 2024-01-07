// Tail recursion 

fn print_reverse(n : i32) {
  if n > 0 {
      println!("{n}");
      print_reverse(n -1);
  } 
}


// Head recursion 

fn print(n : i32) {
    if n > 0 {
        print(n-1);
        println!("{n}");
    }
}

// Tree recursion


fn print_tree(n : i32) {
    if n > 0 {
        println!("{n}");
        print_tree( n-1);
        print_tree( n-1);
    }
}


// Indrect Recursion 


fn func_a(n : i32) {
    if n > 0 {
        println!("{n}");
        func_b(n -1);
    }
}

fn func_b(n : i32) {
    if n > 1 {
        println!("{n}");
        func_a(n/2);
    }
}
//

fn sum(n : i64) -> i64 {
    
    if n == 0 {
       return 0;    
    } else {
        sum(n-1) * n
    }
}


fn main() {
    println!("-----Tail recursion-------");
    print_reverse(3);
    println!("-----Head recursion-------");
    print(3);
    println!("----- Tree recursion -------");
    print_tree(3);
    println!("----- Indirect recursion -------");
    func_a(20);
}
