# frozen_string_literal: true

module AdventOfCode2020
  module Day04
    module Validators
      class Year
        def initialize(first, last)
          @first = first
          @last = last
        end

        def valid?(value)
          return false unless /\A\d{4}\Z/.match?(value)

          (@first..@last).cover?(value.to_i)
        end
      end
    end
  end
end
