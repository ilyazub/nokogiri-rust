# frozen_string_literal: true

RSpec.describe NokogiriRust do
  it 'has a version number' do
    expect(NokogiriRust::VERSION).not_to be nil
  end

  it 'extracts title' do
    big_shopping_html = File.read(File.expand_path('./fixtures/big_shopping.html', __dir__))
    document = NokogiriRust::HTML.parse(big_shopping_html)

    selector = '.eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe'
    title = document.at_css(selector).text

    expect(title).to eq 'Roblox Game eCard [Digital Download]'
  end
end
