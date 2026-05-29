-- name: DARING
-- takes risks

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.3 then
        return "defect"
    else
        return "cooperate"
    end
end
