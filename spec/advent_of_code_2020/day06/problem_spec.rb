# frozen_string_literal: true

require 'advent_of_code_2020/day06/problem'

RSpec.describe AdventOfCode2020::Day06::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  let(:fixture_path) { 'spec/fixtures/day06.txt' }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    it { is_expected.to eq(11) }
  end

  describe '#part_two' do
    subject(:part_two) { problem.part_two }

    it { is_expected.to eq(6) }
  end
end
