fn main() {
    // Suppose we have a 9-digit array. The tail end of the array is calculated
    // as follows:
    //          src digit: 1 2 3 4 5 6 7 8 9
    // digit 9                             +
    // digit 8                           + +
    // digit 7                         + + +
    // digit 6                       + + + +
    // digit 5                     + + + + +
    // -------------------------------------
    // digit 4                   + + + + +
    // digit 3                 + + +       -
    // digit 2               + +     - -   +
    // digit 1             +   -   +   -   +
    //
    // From the ceil(len / 2)th digit to the last digit (i.e. the latter half of
    // the array, or digits 5-9 in this example), the value of a digit is its
    // current value plus the _next_ value of the following digit. Note that the
    // very last value in the array never changes!
    //
    // This means we can calculate the latter half of the array efficiently by
    // calculating values _backward_, using just a single addition per digit.
    // This means that we go from an O(n^2) algorithm with a relatively
    // expensive per-step cost to an O(n) algorithm with a much cheaper
    // per-step cost (basically just an addition plus the cost of taking the
    // last digit.
    //
    // Reddit tells us (https://www.reddit.com/r/adventofcode/comments/ebai4g/2019_day_16_solutions/)
    // that by design, all inputs have a 7-digit skip prefix that happens to
    // identify digits in the last half of the array. Thus, we can just focus
    // on those values exclusively, and use dynamic programming to produce our
    // values.

    let input = "59791911701697178620772166487621926539855976237879300869872931303532122404711706813176657053802481833015214226705058704017099411284046473395211022546662450403964137283487707691563442026697656820695854453826690487611172860358286255850668069507687936410599520475680695180527327076479119764897119494161366645257480353063266653306023935874821274026377407051958316291995144593624792755553923648392169597897222058613725620920233283869036501950753970029182181770358827133737490530431859833065926816798051237510954742209939957376506364926219879150524606056996572743773912030397695613203835011524677640044237824961662635530619875905369208905866913334027160178";
    let skip = input[0..7].parse::<usize>().unwrap();
    let mut digits: Vec<i64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .cycle()
        .take(10000 * input.len())
        .skip(skip)
        .collect();

    for _ in 0..100 {
        for i in (0..(digits.len() - 2)).rev() {
            digits[i] = digits[i] + digits[i + 1];
        }

        // truncate
        for i in 0..digits.len() {
            digits[i] = digits[i].abs() % 10;
        }
    }

    println!(
        "{}",
        digits[0..8]
            .iter()
            .map(|v| v.to_string())
            .collect::<String>()
    );
}
