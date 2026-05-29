-- name: HODGE
-- random strategy with 66% cooperate

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.66 then
        return "cooperate"
    else
        return "defect"
    end
end
