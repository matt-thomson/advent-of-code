# frozen_string_literal: true

RSpec.describe AdventOfCode2020::Day02::Password do
  subject(:password) { described_class.new(input) }

  describe '#valid_for_part_one?' do
    context 'with first example' do
      let(:input) { '1-3 a: abcde' }

      it { is_expected.to be_valid_for_part_one }
    end

    context 'with second example' do
      let(:input) { '1-3 b: cdefg' }

      it { is_expected.not_to be_valid_for_part_one }
    end

    context 'with third example' do
      let(:input) { '2-9 c: ccccccccc' }

      it { is_expected.to be_valid_for_part_one }
    end
  end

  describe '#valid_for_part_two?' do
    context 'with first example' do
      let(:input) { '1-3 a: abcde' }

      it { is_expected.to be_valid_for_part_two }
    end

    context 'with second example' do
      let(:input) { '1-3 b: cdefg' }

      it { is_expected.not_to be_valid_for_part_two }
    end

    context 'with third example' do
      let(:input) { '2-9 c: ccccccccc' }

      it { is_expected.not_to be_valid_for_part_two }
    end
  end
end
