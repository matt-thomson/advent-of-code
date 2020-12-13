# frozen_string_literal: true

require 'advent_of_code_2020/day13/problem'

RSpec.describe AdventOfCode2020::Day13::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    let(:fixture_path) { 'spec/fixtures/day13.txt' }

    it { is_expected.to eq(295) }
  end
end
