use leptos::*;
use ev::Event;

fn calc_avg(array: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for num in array {
        sum += num;
    }
    sum / array.len() as u32
}

fn get_roman_numeral(mut num: u32) -> String {
    let mut numeral = String::new();
    while num > 0 {
        while num >= 1000 {
            numeral = numeral + "M";
            num -= 1000;
        }
        if num >= 900 {
            numeral = numeral + "CM";
            num -= 900
        }
        if num >= 500 {
            numeral = numeral + "D";
            num -= 500;
        }
        if num >= 400 {
            numeral = numeral + "CD";
            num -= 400;
        }
        while num >= 100 {
            numeral = numeral + "C";
            num -= 100;
        }
        if num >= 90 {
            numeral = numeral + "XC";
            num -= 90;
        }
        if num >= 50 {
            numeral = numeral + "L";
            num -= 50;
        }
        if num >= 40 {
            numeral = numeral + "XL";
            num -= 40;
        }
        while num >= 10 {
            numeral = numeral + "X";
            num -= 10;
        }
        if num == 9 {
            numeral = numeral + "IX";
            num -= 9;
        }
        if num >= 5 {
            numeral = numeral + "V";
            num -= 5;
        }
        if num == 4 {
            numeral = numeral + "IV";
            num -= 4;
        }
        while num > 0 {
            numeral = numeral + "I";
            num -= 1;
        }
    }
    numeral.to_string()
}

#[component]
pub fn Algos() -> impl IntoView {
    let avg = create_rw_signal(vec![]);
    let num = create_rw_signal(String::new());
    let roman_numerals = create_rw_signal(vec![]);
    let on_num_input = move |e: Event| {
        let input = event_target_value(&e);
        num.set(input);
    };

    let on_add_to_vec = {
        let avg = avg.clone();
        let num = num.clone();
        move |_| {
            if let Ok(parsed) = num.get().parse::<u32>() {
                let mut new_vec = avg.get();
                let mut new_roman = roman_numerals.get();
                new_vec.push(parsed.clone());
                avg.set(new_vec);
                new_roman.push(get_roman_numeral(parsed));
                roman_numerals.set(new_roman);
            }
        }
    };

    move || {
        view! {
            <div style="
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: space-evenly;
            height: 100vh;
            width: 70vw;
            margin-top: 7vh;">
                <Info />
                <input style="margin: 3%; width: 15vw; text-align: center;" class="material-input" type="number" value={move || num.get()} on:input={on_num_input} />
                <button style="margin: 3%;" class="material-button" on:click={on_add_to_vec}>Add To List</button>
                { move || {
                    if avg.get().len() > 0 {
                        view! {
                            <>
                                <p style="margin: 3%;">Current values: {format!("{:?}", avg.get())}</p>
                                <p style="margin: 3%;">Current avg: {calc_avg(&avg.get())}</p>
                                <p style="margin: 3%;">As Roman Numerals: {format!("{:?}", roman_numerals.get())}</p>
                            </>
                        }
                    } else {
                        view! {
                            <>
                                <p>No numbers added yet</p>
                            </>
                        }
                    }
                }}
            </div>
        }
    }
}

#[component]
fn Info() -> impl IntoView {

    view! {
        <div>
            <h2 style="text-align: center;">Algorithms on Display</h2>
            <p>Input numbers here and I will calculate a running average and the roman numeral representation of the numbers input.</p>
        </div>
    }
}