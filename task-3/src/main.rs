// Topics: simple math

// Ek dafa ka zikar he Pakistan kay wazir e azam kay betay ki ek doctor ne jaan bacahi, 
// wazir e azam ne kaha iskay badaly jo khuwahish he poori ki jayegi. Kion kay doctor 
// ko pata tha wazi e azam unparh hen or ludo kay ilawa inhe kuch nai ata to doctor ne ek 
// ajeeb khuwasih ka izhar kia. Sony kay sikkay or ludo kay her chakor per sony kay sikkon 
// ki tadaad dugni honi chahye.

// Ludo board kay total 92 khanay hoty hen, apka task yeh he kay bataen konsay khanay men 
// kitnay sonay kay sikkay ayengae or poore ludo ke kitne sony kay sikkay banengy.

// Guzarish yeh he kay sab yeh kaam muqarrah waqt per jama kerwaien. Aein Nawzish 🙂

fn main() {
    println!("Gold on box {} is {}",3,check_box(3));
    println!("Total gold {}",total_gold());
}

fn check_box(place: u32) -> u128 {
    2u128.pow(place-1)
}

fn total_gold()-> u128 {
    (1..93).fold(0,|return_value, element| return_value+check_box(element))
}