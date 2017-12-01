# frozen_string_literal: true

require 'day01'

RSpec.describe Day01 do
  it 'solves example 1' do
    expect(described_class.solve('1122')).to eq(3)
  end

  it 'solves example 2' do
    expect(described_class.solve('1111')).to eq(4)
  end

  it 'solves example 3' do
    expect(described_class.solve('1234')).to eq(0)
  end

  it 'solves example 4' do
    expect(described_class.solve('91212129')).to eq(9)
  end
end
