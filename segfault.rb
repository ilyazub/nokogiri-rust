require 'nokogiri_rust'
require 'pry'

big_shopping_html = File.read(File.expand_path('./spec/fixtures/big_shopping.html', __dir__))
document = NokogiriRust::HTML.parse(big_shopping_html)

selector = '.eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe'
title = document.at_css(selector).text

p title
