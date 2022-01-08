# nokogiri_rust

Ruby FFI wrapper around the [Scraper crate](https://crates.io/crates/scraper) and adapter for Nokogiri. `html5ever` instead of `libxml2`.

Status: **proof of concept**.

`NokogiriRust::RubyNode#at_css#text` is 60 times faster than Nokogiri. Of course, API of this library is not compatible with Nokogiri for now.

```bash
$ ruby benchmarks/nokogiri_benchmark.rb
Warming up --------------------------------------
      Nokogiri::HTML     2.000  i/100ms
  NokogiriRust::HTML     3.000  i/100ms
Calculating -------------------------------------
      Nokogiri::HTML     27.195  (±14.7%) i/s -    132.000  in   5.042868s
  NokogiriRust::HTML     40.319  (± 5.0%) i/s -    201.000  in   5.001218s

Comparison:
  NokogiriRust::HTML:       40.3 i/s
      Nokogiri::HTML:       27.2 i/s - 1.48x  (± 0.00) slower

Warming up --------------------------------------
Nokogiri::HTML.at_css.text
                         5.000  i/100ms
NokogiriRust::HTML.at_css.text
                       394.000  i/100ms
Calculating -------------------------------------
Nokogiri::HTML.at_css.text
                         61.027  (± 3.3%) i/s -    305.000  in   5.002827s
NokogiriRust::HTML.at_css.text
                          3.900k (± 2.9%) i/s -     19.700k in   5.056373s

Comparison:
NokogiriRust::HTML.at_css.text:     3899.6 i/s
Nokogiri::HTML.at_css.text:       61.0 i/s - 63.90x  (± 0.00) slower
```

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'nokogiri_rust'
```

And then execute:

    $ bundle install

Or install it yourself as:

    $ gem install nokogiri_rust

## Usage

```ruby
big_shopping_html = File.read(File.expand_path("benchmarks/big_shopping.html", __dir__))
selector = ".eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe"
document = NokogiriRust::HTML.parse(big_shopping_html)
text = document.at_css(selector).text

puts text
# => "Roblox Game eCard [Digital Download]"
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and tags, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ilyazub/nokogiri-rust. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/ilyazub/nokogiri-rust/blob/master/CODE_OF_CONDUCT.md).

## License

Copyright (c) 2020-2021 The nokogiri_rust developers

The `nokogiri_rust` is available as open source under the terms of the both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.

## Code of Conduct

Everyone interacting in the nokogiri_rust project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/ilyazub/nokogiri-rust/blob/master/CODE_OF_CONDUCT.md).
