require "benchmark/ips"
require "nokogiri"
require "nokogiri_rust"

big_shopping_html = File.read(File.expand_path("big_shopping.html", __dir__)).freeze
selector = ".eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe".freeze

Benchmark.ips do |x|
  x.report("Nokogiri::HTML") { Nokogiri::HTML(big_shopping_html) }
  x.report("NokogiriRust::HTML") { NokogiriRust::HTML.parse(big_shopping_html) }

  x.compare!
end

Benchmark.ips do |x|
  nokogiri_document = Nokogiri::HTML(big_shopping_html)
  nokogiri_rust_document = NokogiriRust::HTML.parse(big_shopping_html)

  x.report("Nokogiri::HTML.at_css") { nokogiri_document.at_css(selector).inner_html }
  x.report("NokogiriRust::HTML.at_css") { nokogiri_rust_document.at_css(selector) }

  x.compare!
end
