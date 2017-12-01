# frozen_string_literal: true

require 'day01'

RSpec.describe Day01 do
  describe '.solve_part_one' do
    subject { described_class.solve_part_one(input) }

    context 'with example 1' do
      let(:input) { '1122' }

      it { is_expected.to eq(3) }
    end

    context 'with example 2' do
      let(:input) { '1111' }

      it { is_expected.to eq(4) }
    end

    context 'with example 3' do
      let(:input) { '1234' }

      it { is_expected.to eq(0) }
    end

    context 'with example 4' do
      let(:input) { '91212129' }

      it { is_expected.to eq(9) }
    end
  end

  describe '.solve_part_two' do
    subject { described_class.solve_part_two(input) }

    context 'with example 1' do
      let(:input) { '1212' }

      it { is_expected.to eq(6) }
    end

    context 'with example 2' do
      let(:input) { '1221' }

      it { is_expected.to eq(0) }
    end

    context 'with example 3' do
      let(:input) { '123425' }

      it { is_expected.to eq(4) }
    end

    context 'with example 4' do
      let(:input) { '123123' }

      it { is_expected.to eq(12) }
    end

    context 'with example 5' do
      let(:input) { '12131415' }

      it { is_expected.to eq(4) }
    end
  end
end
