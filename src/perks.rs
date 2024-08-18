use crate::Perk;

pub static INCREASECLICKS1: Perks = Perks{
    name: "2x More Students Per Click",
    price: 1,
    student_rate: 2,
    dps_modifier: 0,
    description: "Adds 2x more Students Per Click"
};

pub static INCREASEFEES1: Perks = Perks{
    name: "Increase student fees by 10%",
    price: 2,
    dps_modifier: 1.1,
    student_rate: 0,
    description: "10% more dollars per second."
};

pub static INCREASECLICKS2: Perks = Perks{
    name: "2x Students per click",
    price: 2,
    student_rate: 2,
    dps_modifier: 0,
    description: "Adds 2x more students per click"
};

pub static INCREASEFEES2: Perks = Perks{
    name: "Increase student fees by 20%",
    price: 3,
    student_rate: 0,
    dps_modifier: 1.2,
    description: "20% more dollars per second"
};

pub static INCREASECLICKS3: Perks = Perks{
    name: "+15 Students per click",
    price: 3,
    student_rate: 2,
    dps_modifier: 0,
    description: "Adds +1 students per click"
};

pub static INCREASEFEES3: Perks = Perks{
    name: "Increase student fees by 20%",
    price: 4,
    student_rate: 0,
    dps_modifier: 1.2,
    description: "20% more dollars per second"
};