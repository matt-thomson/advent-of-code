# frozen_string_literal: true

require 'advent_of_code_2020/day15/problem'

RSpec.describe AdventOfCode2020::Day15::Problem do
  subject(:problem) { described_class.new(*starting_numbers) }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    context 'with first example' do
      let(:starting_numbers) { %w[0 3 6] }

      it { is_expected.to eq(436) }
    end

    context 'with second example' do
      let(:starting_numbers) { %w[1 3 2] }

      it { is_expected.to eq(1) }
    end

    context 'with third example' do
      let(:starting_numbers) { %w[2 1 3] }

      it { is_expected.to eq(10) }
    end

    context 'with fourth example' do
      let(:starting_numbers) { %w[1 2 3] }

      it { is_expected.to eq(27) }
    end

    context 'with fifth example' do
      let(:starting_numbers) { %w[2 3 1] }

      it { is_expected.to eq(78) }
    end

    context 'with sixth example' do
      let(:starting_numbers) { %w[3 2 1] }

      it { is_expected.to eq(438) }
    end

    context 'with seventh example' do
      let(:starting_numbers) { %w[3 1 2] }

      it { is_expected.to eq(1836) }
    end
  end
end
