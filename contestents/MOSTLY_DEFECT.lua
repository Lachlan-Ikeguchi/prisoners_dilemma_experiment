-- name: MOSTLY DEFECT
-- defects with 90% probability

math.randomseed(os.time())

function decide(round, history)
    if math.random() < 0.90 then
        return "defect"
    else
        return "cooperate"
    end
end
