# frozen_string_literal: true

RSpec.describe NokogiriRust do
  it 'has a version number' do
    expect(NokogiriRust::VERSION).not_to be nil
  end

  it 'extracts title' do
    html = File.read(File.expand_path('./fixtures/big_shopping.html', __dir__))
    document = NokogiriRust::HTML.parse(html)

    selector = '.eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe'
    # title = document.at_css(selector).text
    title = NokogiriRust::HTML.parse_and_show_text(html, selector)

    expect(title).to eq 'HP Omen by Obelisk Gaming Desktop Computer, 9th Generation Intel Core i9-9900K Processor, Nvidia GeForce RTX 2080 Super 8 GB, HyperX 32 GB Ram, 1 TB'
  end
end
