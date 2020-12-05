# frozen_string_literal: true

require 'advent_of_code_2020/day03/problem'

RSpec.describe AdventOfCode2020::Day03::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  let(:fixture_path) { 'spec/fixtures/day03.txt' }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    it { is_expected.to eq(7) }
  end

  describe '#part_two' do
    subject(:part_two) { problem.part_two }

    it { is_expected.to eq(336) }
  end
end
