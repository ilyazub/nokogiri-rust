# frozen_string_literal: true

require "bundler/setup"

require "benchmark/ips"
require "nokogiri"
require "nokogiri_rust"

big_shopping_html = File.read(File.expand_path("big_shopping.html", __dir__))
selector = ".eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe"

Benchmark.ips do |x|
  x.report("Nokogiri::HTML") { Nokogiri::HTML(big_shopping_html) }
  x.report("NokogiriRust::HTML") { NokogiriRust::HTML.parse(big_shopping_html) }

  x.compare!
end

Benchmark.ips do |x|
  nokogiri_document = Nokogiri::HTML(big_shopping_html)
  nokogiri_rust_document = NokogiriRust::HTML.parse(big_shopping_html)

  x.report("Nokogiri::HTML.at_css.text") { nokogiri_document.at_css(selector).text }
  x.report("NokogiriRust::HTML.at_css.text") { nokogiri_rust_document.at_css(selector).text }

  x.compare!
end
