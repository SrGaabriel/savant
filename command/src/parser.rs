pub type ParsingResult<T> = Result<T, String>;

pub trait Parseable {
    fn parse(stream: &mut StringStream) -> ParsingResult<Self> where Self: Sized;
}

impl Parseable for String {
    fn parse(stream: &mut StringStream) -> ParsingResult<Self>
    where
        Self: Sized,
    {
        if let Some(next) = stream.next() {
            Ok(next.clone())
        } else {
            Err("No more data".to_string())
        }
    }
}

impl Parseable for i32 {
    fn parse(stream: &mut StringStream) -> ParsingResult<Self>
    where
        Self: Sized,
    {
        if let Some(next) = stream.next() {
            match next.parse::<i32>() {
                Ok(value) => Ok(value),
                Err(_) => Err("Invalid integer".to_string())
            }
        } else {
            Err("No more data".to_string())
        }
    }
}

impl Parseable for bool {
    fn parse(stream: &mut StringStream) -> ParsingResult<Self>
    where
        Self: Sized,
    {
        if let Some(next) = stream.next() {
            match next.as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err("Invalid boolean".to_string())
            }
        } else {
            Err("No more data".to_string())
        }
    }
}

impl<T> Parseable for Option<T>
    where T : Parseable
{
    fn parse(stream: &mut StringStream) -> ParsingResult<Self>
    where
        Self: Sized,
    {
        Ok(T::parse(stream).ok())
    }
}

pub struct StringStream {
    pub data: Vec<String>,
    pub cursor: usize
}

impl StringStream {
    pub fn new(data: Vec<String>) -> Self {
        Self {
            data,
            cursor: 0
        }
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.has_next() {
            let result = Some(&self.data[self.cursor]);
            self.cursor += 1;
            result
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&String> {
        if self.has_next() {
            Some(&self.data[self.cursor])
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cursor >= self.data.len()
    }

    pub fn has_next(&self) -> bool {
        self.cursor < self.data.len()
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
    }
}