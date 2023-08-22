local variables = {
    poop = math.random(),
    balls = "some random string",
    orphan = {status = "unknown"},
    your_mom = "another random string",
    gay = {isGay = false},
    fuck = {isBad = true}
}

local function balls()
    for i = 1, 100 do
        if i % 3 == 0 and i % 5 == 0 then
            print("FizzBuzz")
        elseif i % 3 == 0 then
            print("Fizz")
        elseif i % 5 == 0 then
            print("Buzz")
        else
            print(i)
        end
    end
end

local function aborted()
    return variables.orphan.status == "aborted"
end

local function your_mom()
    if variables.your_mom and variables.your_mom ~= "" then
        if variables.gay.isGay then
            fuck()
        else
            print("Nothing to see here.")
        end
    end
end

local function fuck()
    if variables.fuck.isBad then
        print("you're gay")
    else
        print("This shouldn't be printed.")
    end
end

local function main()
    if variables.poop then
        balls()
    end

    if aborted() then
        repeat
            print("Repeating this useless statement.")
        until not aborted()
    end

    your_mom()
end

main()
