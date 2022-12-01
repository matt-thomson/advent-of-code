# frozen_string_literal: true

require 'advent_of_code_2020/day07/problem'

RSpec.describe AdventOfCode2020::Day07::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  let(:fixture_path) { 'spec/fixtures/day07a.txt' }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    it { is_expected.to eq(4) }
  end

  describe '#part_two' do
    subject(:part_two) { problem.part_two }

    it { is_expected.to eq(32) }

    context 'with second example' do
      let(:fixture_path) { 'spec/fixtures/day07b.txt' }

      it { is_expected.to eq(126) }
    end
  end
end
