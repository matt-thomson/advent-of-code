# frozen_string_literal: true

require 'advent_of_code_2020/day04/problem'

RSpec.describe AdventOfCode2020::Day04::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  let(:fixture_path) { 'spec/fixtures/day04.txt' }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    it { is_expected.to eq(2) }
  end
end
