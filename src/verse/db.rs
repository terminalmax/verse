use rusqlite::{Connection, OpenFlags, Result};
use std::env;

#[derive(Copy, Clone, PartialEq)]
pub enum BibleBook {
    Genesis = 1,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    FirstSamuel,
    SecondSamuel,
    FirstKings,
    SecondKings,
    FirstChronicles,
    SecondChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    FirstCorinthians,
    SecondCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    FirstThessalonians,
    SecondThessalonians,
    FirstTimothy,
    SecondTimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    FirstPeter,
    SecondPeter,
    FirstJohn,
    SecondJohn,
    ThirdJohn,
    Jude,
    Revelation,
}

impl BibleBook {
    pub fn as_str(&self) -> &'static str {
        match self {
            BibleBook::Genesis => "Genesis",
            BibleBook::Exodus => "Exodus",
            BibleBook::Leviticus => "Leviticus",
            BibleBook::Numbers => "Numbers",
            BibleBook::Deuteronomy => "Deuteronomy",
            BibleBook::Joshua => "Joshua",
            BibleBook::Judges => "Judges",
            BibleBook::Ruth => "Ruth",
            BibleBook::FirstSamuel => "1 Samuel",
            BibleBook::SecondSamuel => "2 Samuel",
            BibleBook::FirstKings => "1 Kings",
            BibleBook::SecondKings => "2 Kings",
            BibleBook::FirstChronicles => "1 Chronicles",
            BibleBook::SecondChronicles => "2 Chronicles",
            BibleBook::Ezra => "Ezra",
            BibleBook::Nehemiah => "Nehemiah",
            BibleBook::Esther => "Esther",
            BibleBook::Job => "Job",
            BibleBook::Psalms => "Psalms",
            BibleBook::Proverbs => "Proverbs",
            BibleBook::Ecclesiastes => "Ecclesiastes",
            BibleBook::SongOfSolomon => "Song of Solomon",
            BibleBook::Isaiah => "Isaiah",
            BibleBook::Jeremiah => "Jeremiah",
            BibleBook::Lamentations => "Lamentations",
            BibleBook::Ezekiel => "Ezekiel",
            BibleBook::Daniel => "Daniel",
            BibleBook::Hosea => "Hosea",
            BibleBook::Joel => "Joel",
            BibleBook::Amos => "Amos",
            BibleBook::Obadiah => "Obadiah",
            BibleBook::Jonah => "Jonah",
            BibleBook::Micah => "Micah",
            BibleBook::Nahum => "Nahum",
            BibleBook::Habakkuk => "Habakkuk",
            BibleBook::Zephaniah => "Zephaniah",
            BibleBook::Haggai => "Haggai",
            BibleBook::Zechariah => "Zechariah",
            BibleBook::Malachi => "Malachi",
            BibleBook::Matthew => "Matthew",
            BibleBook::Mark => "Mark",
            BibleBook::Luke => "Luke",
            BibleBook::John => "John",
            BibleBook::Acts => "Acts",
            BibleBook::Romans => "Romans",
            BibleBook::FirstCorinthians => "1 Corinthians",
            BibleBook::SecondCorinthians => "2 Corinthians",
            BibleBook::Galatians => "Galatians",
            BibleBook::Ephesians => "Ephesians",
            BibleBook::Philippians => "Philippians",
            BibleBook::Colossians => "Colossians",
            BibleBook::FirstThessalonians => "1 Thessalonians",
            BibleBook::SecondThessalonians => "2 Thessalonians",
            BibleBook::FirstTimothy => "1 Timothy",
            BibleBook::SecondTimothy => "2 Timothy",
            BibleBook::Titus => "Titus",
            BibleBook::Philemon => "Philemon",
            BibleBook::Hebrews => "Hebrews",
            BibleBook::James => "James",
            BibleBook::FirstPeter => "1 Peter",
            BibleBook::SecondPeter => "2 Peter",
            BibleBook::FirstJohn => "1 John",
            BibleBook::SecondJohn => "2 John",
            BibleBook::ThirdJohn => "3 John",
            BibleBook::Jude => "Jude",
            BibleBook::Revelation => "Revelation",
        }
    }

    #[inline]
    pub fn max_chapter_count(&self) -> u8 {
        Self::CHAPTER_COUNT[*self as usize]
    }

    const CHAPTER_COUNT: [u8; 67] = [
        0,   // Invalid
        50,  // Genesis
        40,  // Exodus
        27,  // Leviticus
        36,  // Numbers
        34,  // Deuteronomy
        24,  // Joshua
        21,  // Judges
        4,   // Ruth
        31,  // 1 Samuel
        24,  // 2 Samuel
        22,  // 1 Kings
        25,  // 2 Kings
        29,  // 1 Chronicles
        36,  // 2 Chronicles
        10,  // Ezra
        13,  // Nehemiah
        10,  // Esther
        42,  // Job
        150, // Psalms
        31,  // Proverbs
        12,  // Ecclesiastes
        8,   // Song of Solomon
        66,  // Isaiah
        52,  // Jeremiah
        5,   // Lamentations
        48,  // Ezekiel
        12,  // Daniel
        14,  // Hosea
        3,   // Joel
        9,   // Amos
        1,   // Obadiah
        4,   // Jonah
        7,   // Micah
        3,   // Nahum
        3,   // Habakkuk
        2,   // Zephaniah
        14,  // Haggai
        8,   // Zechariah
        4,   // Malachi
        28,  // Matthew
        16,  // Mark
        24,  // Luke
        21,  // John
        28,  // Acts
        16,  // Romans
        16,  // 1 Corinthians
        13,  // 2 Corinthians
        6,   // Galatians
        6,   // Ephesians
        4,   // Philippians
        4,   // Colossians
        5,   // 1 Thessalonians
        3,   // 2 Thessalonians
        6,   // 1 Timothy
        4,   // 2 Timothy
        3,   // Titus
        1,   // Philemon
        13,  // Hebrews
        5,   // James
        5,   // 1 Peter
        3,   // 2 Peter
        5,   // 1 John
        1,   // 2 John
        1,   // 3 John
        1,   // Jude
        22,  // Revelation
    ];
}

pub struct Database {
    connection: Connection,
    cache: Vec<Vec<String>>,
    pub book: BibleBook,
}

impl Database {
    pub fn connect_and_load(book: BibleBook) -> Result<Self, ()> {
        let path = env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(|dir| dir.join("ASV.db")))
            .expect("Could not get path to dir.");

        let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .expect("Could not connect to DB. The DB has to be in the same dir as the executable.");

        let mut db = Database {
            connection,
            cache: vec![],
            book,
        };
        db.load_book(book)?;

        Ok(db)
    }

    pub fn get_chapter(&self, num: u8) -> Option<&Vec<String>> {
        if self.book.max_chapter_count() >= num && num > 0 {
            return Some(&(self.cache[num as usize - 1]));
        }

        None
    }

    pub fn load_book(&mut self, book: BibleBook) -> Result<(), ()> {
        self.cache = vec![];
        self.book = book;

        let mut stmt = self
            .connection
            .prepare_cached("SELECT text FROM ASV_verses WHERE book_id = ? AND chapter = ?")
            .expect("Couldn't prepare statement");

        for chapter in 1..=self.book.max_chapter_count() {
            let verses: Vec<String> = stmt
                .query_map([book as i32, chapter as i32], |row| row.get(0))
                .map_err(|_| ())? // Extract `text` column
                .collect::<Result<Vec<String>, _>>()
                .map_err(|_| ())?;

            self.cache.push(verses);
        }

        Ok(())
    }
}
