fn main() {
    const SONG_LYRICS: [&str; 12] = [
      "a Partridge in a Pear Tree!",
      "Two Turtle Doves",
      "Three French hens",
      "Four calling birds",
      "FIVE GOLDEN RINGS",
      "Six geese a-laying",
      "Seven swans a-swimming",
      "Eight maids a-milking",
      "Nine ladies dancing",
      "Ten lords a-leaping",
      "Eleven pipers piping",
      "Twelve drummers drumming"
    ];

    const WRITTEN_NUMBERS: [&str; 12] = [
      "first",
      "second",
      "third",
      "fourth",
      "fifth",
      "sixth",
      "seventh",
      "eighth",
      "ninth",
      "tenth",
      "eleventh",
      "twelfth"
    ];

    let mut end_of_song = String::new();

    for number in 0..12 {

        print!("\rOn the {} day of Christmas,\nMy true love gave to me", WRITTEN_NUMBERS[number]);

        if number == 0 {
            end_of_song = SONG_LYRICS[number].to_string();
        } else if number == 1 {
            end_of_song = SONG_LYRICS[number].to_owned() + ",\nAnd " + &end_of_song;
        } else {
            end_of_song = SONG_LYRICS[number].to_owned() + ",\n" + &end_of_song;
        };

        print!("\n{} \n\n", end_of_song);
    };
}
