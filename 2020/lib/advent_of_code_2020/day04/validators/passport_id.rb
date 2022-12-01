# frozen_string_literal: true

module AdventOfCode2020
  module Day04
    module Validators
      class PassportID
        def valid?(value)
          /\A\d{9}\Z/.match?(value)
        end
      end
    end
  end
end
