-- name: LARRY
-- random strategy with 75% cooperate

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.75 then
        return "cooperate"
    else
        return "defect"
    end
end
