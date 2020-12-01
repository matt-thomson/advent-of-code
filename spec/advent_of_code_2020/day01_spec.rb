# frozen_string_literal: true

RSpec.describe AdventOfCode2020::Day01 do
  let(:fixture_path) { 'spec/fixtures/day01.txt' }
  subject(:day01) { described_class.new(fixture_path) }

  describe '#part_one' do
    subject(:part_one) { day01.part_one }

    it { is_expected.to eq(514_579) }
  end

  describe '#part_two' do
    subject(:part_two) { day01.part_two }

    it { is_expected.to eq(241_861_950) }
  end
end
