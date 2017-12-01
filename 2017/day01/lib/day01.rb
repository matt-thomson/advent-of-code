module Day01
  def self.solve(input)
    digits = input.split(//).map(&:to_i)
    digits << digits[0]

    digits.each_cons(2)
          .select { |x, y| x == y }
          .map { |pair| pair[0] }
          .inject(0) { |sum, x| sum + x }
  end
end
