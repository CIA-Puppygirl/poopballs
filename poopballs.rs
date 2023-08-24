fn balls(poop_num: u8, mut balls_num: u8) -> u8 {
    balls_num += poop_num;
    println!("balls incremented by {}, now {}", poop_num, balls_num);
    return balls_num;
}

fn poop(balls_num: u8, mut poop_num: u8) -> u8 {
    poop_num += balls_num;
    println!("poop incremented by {}, now {}", balls_num, poop_num);
    return poop_num;
}

fn abort(_orphan: u8) -> u8 {
    return 0;
}

fn fuck(mom: &str) {
    println!("your mom({}) has been fucked and is now pregnant", mom);
}

fn conversion_therapy<'life>() -> &'life str {
    println!("your mom gay");
    return "gay";
}

fn main() {
    let mut poop_num: u8 = 2;
    let mut balls_num: u8 = 1;
    let mut orphan: u8 = 0;
    let mut your_mom: &str = "straight";

    while poop_num < 233 {
        if poop_num > balls_num {
            balls_num = balls(poop_num, balls_num);
        } else if balls_num > poop_num {
            poop_num = poop(balls_num, poop_num);
        }
    }
   

    if orphan > 0 {
        println!("there is an orphan child");
        orphan = abort(orphan);
        println!("orphan aborted");
        if your_mom == "gay" {
            fuck(your_mom);
            orphan += 1;
            println!("{} orphan child born", orphan);
        } else {
            println!("your mom straight");
            your_mom = conversion_therapy();
            fuck(your_mom);
            orphan += 1;
            println!("{} orphan child born", orphan);
        }
    } else {
        if your_mom == "gay" {
            fuck(your_mom);
            orphan += 1;
            println!("{} orphan child born", orphan);
        } else {
            println!("your mom straight");
            your_mom = conversion_therapy();
            fuck(your_mom);
            orphan += 1;
            println!("{} orphan child born", orphan);
        }
    }
}
