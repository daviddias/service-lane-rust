use std::io;

fn main() {
    let mut reader = io::stdin();

    let first_line: Vec<int> = 
        reader.read_line().unwrap().as_slice()  
              .split(' ').map(|s| s.trim())     
              .filter(|s| !s.is_empty())        
              .map(|s| from_str(s).unwrap())    
              .collect();                       

    let n: int = first_line[0];
    let t: int = first_line[1];

    // println!("N {}", n);
    // println!("T {}", t);

    let lane: Vec<uint> = 
        reader.read_line().unwrap().as_slice()  // (1)
              .split(' ').map(|s| s.trim())     // (2)
              .filter(|s| !s.is_empty())        // (3)
              .map(|s| from_str(s).unwrap())    // (4)
              .collect();                       // (5)  

    // println!("LANE {}", lane);

    for _ in range(0, t) {
        let test: Vec<uint> = 
            reader.read_line().unwrap().as_slice()  // (1)
                  .split(' ').map(|s| s.trim())     // (2)
                  .filter(|s| !s.is_empty())        // (3)
                  .map(|s| from_str(s).unwrap())    // (4)
                  .collect();                       // (5) 

        // println!("test {}", test);  

        println!("{}", belly(lane.clone(), test[0], test[1]));          
    }
}

fn belly(lane: Vec<uint>, start:uint, end:uint) -> int {
    let mut width:int = 3;

    for i in range(0u, end-start+1) {
        if lane[start+i] == 1 {
            return 1;
        }
        if lane[start+i] == 2 {
            width = 2;
        }
    }
    return width;
}