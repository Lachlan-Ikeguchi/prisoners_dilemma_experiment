-- name: LEMMING
-- follows the crowd: does what the majority of recent opponent moves were

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    local coop = 0
    local defect = 0
    local window = math.min(10, #history)
    local start = #history - window + 1
    
    for i = start, #history do
        if history[i] == "cooperate" then coop = coop + 1
        else defect = defect + 1 end
    end
    
    if coop >= defect then
        return "cooperate"
    else
        return "defect"
    end
end
