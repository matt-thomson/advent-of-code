# frozen_string_literal: true

RSpec.describe AdventOfCode2020::Day04::Document do
  subject(:password) { described_class.new(input) }

  describe '#valid_structure?' do
    context 'with first example' do
      let(:input) do
        <<~DOCUMENT
          ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
          byr:1937 iyr:2017 cid:147 hgt:183cm
        DOCUMENT
      end

      it { is_expected.to be_valid_structure }
    end

    context 'with second example' do
      let(:input) do
        <<~DOCUMENT
          iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
          hcl:#cfa07d byr:1929
        DOCUMENT
      end

      it { is_expected.not_to be_valid_structure }
    end

    context 'with third example' do
      let(:input) do
        <<~DOCUMENT
          hcl:#ae17e1 iyr:2013
          eyr:2024
          ecl:brn pid:760753108 byr:1931
          hgt:179cm
        DOCUMENT
      end

      it { is_expected.to be_valid_structure }
    end

    context 'with fourth example' do
      let(:input) do
        <<~DOCUMENT
          hcl:#cfa07d eyr:2025 pid:166559648
          iyr:2011 ecl:brn hgt:59in
        DOCUMENT
      end

      it { is_expected.not_to be_valid_structure }
    end
  end
end
