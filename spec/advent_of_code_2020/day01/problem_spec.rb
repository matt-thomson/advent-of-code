# frozen_string_literal: true

require 'advent_of_code_2020/day01/problem'

RSpec.describe AdventOfCode2020::Day01::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  let(:fixture_path) { 'spec/fixtures/day01.txt' }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    it { is_expected.to eq(514_579) }
  end

  describe '#part_two' do
    subject(:part_two) { problem.part_two }

    it { is_expected.to eq(241_861_950) }
  end
end
