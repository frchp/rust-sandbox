use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    numbers();
    strings();
    tableaux();
    vecteurs();
    tuples();
    boucles();
    fonctions();
}

fn numbers() {
    // let val1: i32 = 4;
    // let val2 = 5;
    // let val3 = 6usize;
    // let val_archi = val1 as usize + val2 as usize;
    // let mut val_archi = val3 + val2 as usize;
    // println!("Numbers {} {val_archi}", val_archi);

    let mut a = 5;
    println!("Numbers {} {a}", a);
    a = 6;

    // error[E0384]: cannot assign twice to immutable variable `a`
    // let a = 5;
    // a = 6;

    println!("Numbers {} {a}", a);
}

fn strings() {
    // type saves pointer, max capacity, and length of array
    let type_string = String::from("ma string");
    // type saves pointer and only preallocated memory for given string
    let type_str = "ma str";

    let mut ma_format = format!("ceci est une string avec ma_str {type_str}");
    ma_format.push_str("!!!");
    ma_format.push('!');

    let ma_string_format_str = type_string.as_str();

    let mon_caractere = type_string.chars().nth(1).unwrap();
    for lettre in ma_string_format_str.chars() {
        println!("lettre {} {lettre}", mon_caractere);
    }
}

fn tableaux() {
    let tableau_statique: [i32; 4] = [0, 1, 2, 3];
    let tableau_dynamique: &[i32] = &tableau_statique[..]; // debut à la fin
    // let tableau_dynamique: &[i32] = &tableau_statique[1..3]; // element 1 a 3
    println!("tableau_dynamique => {tableau_dynamique:#?}");

    let mut tableau = [0, 1, 2, 3];
    let slice: &mut [i32] = &mut tableau[..];
    slice.fill(42);
    println!("tableau => {tableau:#?}");
    println!("est égal ? {}", tableau == [1, 2, 3, 4]);

    for element in tableau {
        println!("element {}", element);
    }
}

fn vecteurs() {
    // tableau dynamique
    let mut vec1: Vec<usize> = vec![0usize; 5];
    let vec2: Vec<usize> = vec![1,2,3,4];
    println!("egalite ? {}", vec1 == vec2); // egalite possible entre vec de taille différentes

    println!("vec1 avant dedup {:?}", vec1);
    vec1.dedup();
    println!("vec1 après dedup {:?}", vec1);
    vec1.push(5);
    vec1.push(5);
    vec1.push(5);
    vec1.push(5);
    vec1.push(6);
    println!("vec1 après push {:?}", vec1);

    // iteration sur vecteur
    println!("vec2 {:#?}", vec2);
    for mon_element in vec2.iter().skip(2).rev() {
        // skip les 2 premiers elements
        // rev pour reverse le vectuer
        println!("mon element {mon_element}");
    }

    // dictionnaire : string and value
    let mut animaux: HashMap<String, i32> = HashMap::new();
    animaux.insert("chien".to_string(), 42);
    animaux.insert("chat".to_string(), 3);

    println!(
        "nombre de chats = {}",
        animaux.get(&"chat".to_string()).unwrap()
    );

    for (animal, nombre) in animaux {
        println!("animal '{animal}' = {nombre}");
    }
}

fn tuples() {
    let mon_tuple = (1, 3);
    let mon_autre_tuple: (usize, String, &str) = (42, "chats".to_string(), "Paris");
    let tuple_un_element = (8,);
    let tuple_vide = ();

    println!(
        "nombre de {} qui vivent à {} = {}",
        mon_autre_tuple.1, mon_autre_tuple.2, mon_autre_tuple.0
    );

    for (index, element) in [1, 2, 3].iter().enumerate() {
        println!("element index {index} = {element}");
    }
}

fn boucles() {
    let tableau = [0, 1, 2, 3];
    // boucle for in avec range
    for index in 0..=3 { // 0..3 ira jusqua 2 seulement
        println!(
            "boucle avec range, element index {index} = {}",
            tableau[index]
        );
    }

    // boucle loop
    let mut index = 0;
    loop {
        if index >= tableau.len() {
            break;
        }
        println!("element avec boucle loop, {index} = {}", tableau[index]);

        index += 1; // ++ n'existe pas
    }

    // boucle while
    let mut index = 0;
    while index < tableau.len() {
        println!("element avec boucle while, {index} = {}", tableau[index]);
        index += 1;
    }
}

fn fonctions() {
    let somme = additionne(5, 20);
    let division = divise(5, 0);

    // {:?} mode debug
    println!("double tuple -- {:?}", double_tuple((5, 255)));

    let reponse = execute(additionne);
    // quick and dirty debugging
    dbg!(reponse);
}

fn additionne(left: i32, right: i32) -> i32 {
    // Tail expression => pas de return, sans ; pour montrer le return
    left + right
}

fn divise(left: usize, right: usize) -> f64 {
    if right == 0 {
        return 0.0;
    }

    left as f64 / right as f64
}

fn double_tuple(val: (usize, i32)) -> (usize, i32) {
    (val.0 * 2, val.1 * 2)
}

// pub fn name(name_arg: type) -> type_ret /* pub facultative */
fn execute(fonction: fn(i32, i32) -> i32) -> i32 {
    fonction(1, 3)
}