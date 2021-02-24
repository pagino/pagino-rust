const FIRST: i32 = -1;
const PREVIOUS: i32 = -2;
const START_ELLIPSIS: i32 = -3;
const END_ELLIPSIS: i32 = -4;
const NEXT: i32 = -5;
const LAST: i32 = -6;

pub struct Pagino {
  pub show_first: bool,
  pub show_previous: bool,
  pub show_next: bool,
  pub show_last: bool,
  pub page: i32,
  pub count: i32,
  pub sibling_count: i32,
  pub boundary_count: i32,
}

impl Pagino {
  pub fn new(
    show_first: bool,
    show_previous: bool,
    show_next: bool,
    show_last: bool,
    page: i32,
    count: i32,
    sibling_count: i32,
    boundary_count: i32,
  ) -> Pagino {
    Pagino {
      show_first,
      show_previous,
      show_next,
      show_last,
      page,
      count,
      sibling_count,
      boundary_count,
    }
  }

  pub fn set_count(&mut self, count: i32) -> () {
    self.count = count;
  }

  pub fn set_page(&mut self, page: i32) -> () {
    if page > 0 && page <= self.count {
      self.page = page;
    }
  }

  pub fn first(&mut self) -> () {
    self.set_page(1)
  }

  pub fn last(&mut self) -> () {
    self.set_page(self.count)
  }

  pub fn next(&mut self) -> () {
    self.set_page(self.page + 1)
  }

  pub fn previous(&mut self) -> () {
    self.set_page(self.page - 1)
  }

  pub fn get_pages(&self) -> Vec<i32> {
    let start_pages: Vec<i32> = create_start_pages(self.boundary_count, self.count);
    let end_pages: Vec<i32> = create_end_pages(self.boundary_count, self.count);

    let sibling_start: i32 = create_siblings_start(
      self.boundary_count,
      self.count,
      self.page,
      self.sibling_count,
    );

    let sibling_end: i32 = create_siblings_end(
      self.boundary_count,
      self.count,
      self.page,
      self.sibling_count,
      &end_pages,
    );

    let mut pages: Vec<i32> = Vec::new();

    if self.show_first {
      pages.push(FIRST);
    }

    if self.show_previous {
      pages.push(PREVIOUS);
    }

    for x in start_pages {
      pages.push(x);
    }

    println!(
      "sibling_start {} boundary_count {}",
      sibling_start,
      self.boundary_count + 2
    );
    if sibling_start > self.boundary_count + 2 {
      pages.push(START_ELLIPSIS);
    } else if self.boundary_count + 1 < self.count - self.boundary_count {
      pages.push(self.boundary_count + 1)
    }

    for x in create_range(sibling_start, sibling_end) {
      pages.push(x);
    }

    if sibling_end < self.count - self.boundary_count - 1 {
      pages.push(END_ELLIPSIS);
    } else if self.count - self.boundary_count > self.boundary_count {
      pages.push(self.count - self.boundary_count);
    }

    for x in end_pages {
      pages.push(x);
    }

    if self.show_next {
      pages.push(NEXT);
    }

    if self.show_last {
      pages.push(LAST);
    }

    pages
  }
}

fn min(a: i32, b: i32) -> i32 {
  if a < b {
    a
  } else {
    b
  }
}

fn max(a: i32, b: i32) -> i32 {
  if a > b {
    a
  } else {
    b
  }
}

fn create_range(start: i32, end: i32) -> Vec<i32> {
  let length: i32 = end - start + 1;
  let mut vec: Vec<i32> = Vec::new();
  let mut count = 0;
  while count < length {
    vec.push(start + count);
    count += 1;
  }

  vec
}

fn create_start_pages(boundary_count: i32, count: i32) -> Vec<i32> {
  create_range(1, min(boundary_count, count))
}

fn create_end_pages(boundary_count: i32, count: i32) -> Vec<i32> {
  create_range(max(count - boundary_count + 1, boundary_count + 1), count)
}

fn create_siblings_start(boundary_count: i32, count: i32, page: i32, sibling_count: i32) -> i32 {
  max(
    min(
      page - sibling_count,
      count - boundary_count - (sibling_count * 2) - 1,
    ),
    boundary_count + 2,
  )
}

fn create_siblings_end(
  boundary_count: i32,
  count: i32,
  page: i32,
  sibling_count: i32,
  end_pages: &Vec<i32>,
) -> i32 {
  let end_element = if end_pages.len() > 0 {
    end_pages[0] - 2
  } else {
    count - 1
  };

  min(
    max(
      page + sibling_count,
      boundary_count + (sibling_count * 2) + 2,
    ),
    end_element,
  )
}
